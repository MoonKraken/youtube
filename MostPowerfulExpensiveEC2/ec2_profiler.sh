#!/bin/sh

# helpful for looking at GPU usage:
# nvidia-smi --query-gpu=temperature.gpu,utilization.gpu,utilization.memory,memory.total,memory.free,memory.used --format=csv -l 5

# $1 should be the s3 prefix we upload results to
# $2 should be the s3 path to the blender project zip file
# $3 should be the zip file name to extract (should we derive from $2?)
# $4 is the path to the blender file to render after extraction
# $5 is the path to the blender archive
# $6 is the filename of the blender archive
export INSTANCE_TYPE=$(ec2-metadata --instance-type | awk '{print $2}')
echo $INSTANCE_TYPE > instance_type.txt
aws s3 cp ./instance_type.txt $1/$INSTANCE_TYPE/

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
aws s3 cp $2 .
unzip $3

# blender render

# the following doesnt seem to be necessary on the deep learning image 
#sudo yum -y install libX11 libXi libXxf86vm libXfixes libXrender libGL

# A10 driver download link
# https://us.download.nvidia.com/tesla/470.82.01/NVIDIA-Linux-x86_64-470.82.01.run
mkdir blenderOutput
blender -b $4 -a --enable-autoexec --cycles-device 'CUDA+CPU' --cycles-print-stats -o ~/blenderOutput

#ex
#blender-2.93.5-linux-x64/blender -b 06_035_A/06_035_A.lighting.blend -noaudio -s 160 -e 162 --enable-autoexec -o ~/blenderOutput -a -- --cycles-device 'CUDA' --cycles-print-stats

aws s3 cp ~/blenderOutput $1/$INSTANCE_TYPE/blenderOutput --recursive