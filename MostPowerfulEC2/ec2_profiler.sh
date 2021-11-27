#!/bin/sh

# helpful for looking at GPU usage:
# nvidia-smi --query-gpu=temperature.gpu,utilization.gpu,utilization.memory,memory.total,memory.free,memory.used --format=csv -l 5

# $1 should be the s3 prefix we upload results to
# $2 should be the s3 path to the blender project zip file
# $3 should be the zip file name to extract (should we derive from $2?)
# $4 is the path to the blender file to render after extraction
# $5 is the path to the blender archive
# $6 is the filename of the blender archive
# $7 is the blender executable path after extraction

echo 'Arguments passed: '$@

export INSTANCE_TYPE=$(ec2-metadata --instance-type | awk '{print $2}')
echo $INSTANCE_TYPE > instance_type.txt
aws s3 cp ./instance_type.txt $1/$INSTANCE_TYPE/

mkdir /home/ec2-user/workspace
chown ec2-user /home/ec2-user/workspace

INSTANCE_PREFIX=${INSTANCE_TYPE:0:2}
# g5 and p4 instances have instance storage we can use
if [ $INSTANCE_PREFIX = "g5" ] || [ $INSTANCE_PREFIX = "p4" ]
then
    # mount the fast nvme drive into the workspace dir
    mkfs -t xfs /dev/nvme1n1
    mount /dev/nvme1n1 /home/ec2-user/workspace
fi

# do everything in workspace
cd /home/ec2-user/workspace

# the following are necessary for blender to run
yum -y install libX11 libXi libXxf86vm libXfixes libXrender libGL

mkdir blenderOutput
LOG_FILE='./blenderOutput/log.txt'
# download nvidia drivers if instance is p or g,
# but use the public drivers if we are on p4
INSTANCE_FIRST_CHAR=${INSTANCE_TYPE:0:1}
if [ $INSTANCE_FIRST_CHAR = "g" ] || [ $INSTANCE_FIRST_CHAR = "p" ]
then
    if [ $INSTANCE_FIRST_CHAR = "p" ]
    then
        wget 'https://us.download.nvidia.com/tesla/470.82.01/NVIDIA-Linux-x86_64-470.82.01.run'
        echo "Using public nvidia driver" >> $LOG_FILE
    else
        aws s3 cp --recursive s3://ec2-linux-nvidia-drivers/latest/ .
        echo "Using GRID nvidia driver" >> $LOG_FILE
    fi

    # GRID driver download instructions
    # from https://docs.aws.amazon.com/AWSEC2/latest/UserGuide/install-nvidia-driver.html#nvidia-GRID-driver
    # the initial yum update doesnt appear to be necessary when using the latest ami
    yum install -y gcc kernel-devel-$(uname -r)
    chmod +x NVIDIA-Linux-x86_64*.run
    /bin/sh ./NVIDIA-Linux-x86_64*.run -s
else
    echo "Non-GPU instance detected, skipping nvidia driver install" >> $LOG_FILE
fi

# grab blender
# ex:
# aws s3 cp s3://powerful-expensive-ec2/Blender/blender-2.93.5-linux-x64.tar.xz .
# tar -xvf blender-2.93.5-linux-x64.tar.xz
aws s3 cp $5 .
tar -xvf $6

# grab project file
# ex: 
# aws s3 cp s3://powerful-expensive-ec2/BlenderProjects/06_035_a.zip .
# unzip 06_035_a.zip
# aws s3 cp s3://powerful-expensive-ec2/BlenderProjects/05_025_a.zip .
# unzip 05_025_a.zip
aws s3 cp $2 .
unzip $3

mkdir blenderOutput
if [ $INSTANCE_FIRST_CHAR = "g" ] || [ $INSTANCE_FIRST_CHAR = "p" ]
then
    CYCLES_DEVICE='OPTIX+CPU'
else
    CYCLES_DEVICE='CPU'
fi

echo "Cycles device: $CYCLES_DEVICE" >> $LOG_FILE

$7 -b $4 -noaudio --enable-autoexec -x 1 -F PNG -o './blenderOutput/frame_#####' -f 203 -- --cycles-device $CYCLES_DEVICE > ./blenderOutput/blenderRun.txt

#ex
#blender-2.93.5-linux-x64/blender -b 06_035_A/06_035_A.lighting.blend -noaudio -s 160 -e 162 --enable-autoexec -o blenderOutput/frame_##### -a -- --cycles-device 'OPTIX+CPU'
#blender-2.93.5-linux-x64/blender -b 05_025_A/05_025_A.lighting.blend -noaudio -s 160 -e 162 --enable-autoexec -o blenderOutput/frame_##### -a -- --cycles-device 'OPTIX+CPU'

aws s3 cp ./blenderOutput $1/$INSTANCE_TYPE/blenderOutput --recursive