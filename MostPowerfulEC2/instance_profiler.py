import boto3
from datetime import datetime
import time

shell_script = 'file path'
job_finished_file_path = '~/profiling_completed'
bucket_name = 'powerful-expensive-ec2'
local_script_path = './'
script_name = 'ec2_profiler.sh'
script_url = 's3://'+bucket_name+'/'+script_name

# this is used for the s3 prefix for the results
run_timestamp = datetime.now().isoformat()
print("Run timestamp: " +run_timestamp)
upload_prefix = 's3://'+bucket_name+'/'+run_timestamp

blender_project_archive_filename = '06_035_a.zip'
blender_project_archive_s3_path = 's3://'+bucket_name+'/BlenderProjects/'+blender_project_archive_filename
blender_project_path = '06_035_A/06_035_A.lighting.blend'
blender_dir_name = 'blender-2.93.5-linux-x64'
blender_archive_filename = blender_dir_name+'.tar.xz'
blender_executable_path = blender_dir_name + '/' + 'blender'
blender_archive_s3_path = 's3://'+ bucket_name + '/Blender/' + blender_archive_filename

# first upload the script to s3
s3_client = boto3.client('s3')
response = s3_client.upload_file(
    local_script_path+script_name, 
    bucket_name, 
    script_name
)

print("file uploaded")
print(response)
ec2 = boto3.resource('ec2')

instances_to_profile = [
    'g5.4xlarge'
]
instance_ids = []

for instance_type in instances_to_profile:
    # scp the shell script to run
    # create a new EC2 instance
    instance_ids.append(ec2.create_instances(
        ImageId='ami-00be885d550dcee43',
        MinCount=1,
        MaxCount=1,
        KeyName='ken_profiling_pair',
        SecurityGroupIds=[
            'sg-09a856b5b0947de6e',
        ],
        InstanceType=instance_type,
        IamInstanceProfile={
            'Name': 'AmazonSSMRoleForInstancesQuickSetup'
        },
        # this is only because some instances dont
        # come with instance storage, and the default
        # EBS volume size is 8GB which is insufficient
        BlockDeviceMappings=[
            {
                'DeviceName':'/dev/xvda',
                'Ebs': {
                    'DeleteOnTermination': True,
                    'VolumeSize': 30,
                    'VolumeType': 'gp2',
                    'Encrypted': False
                }
            }
        ]
    )[0].id)

ec2_client = boto3.client('ec2')

# loop until all of our newly created instances
# are present and in a running state
while True:
    instance_state_dict = {}
    describe_result = ec2_client.describe_instances()
    for reservation in describe_result['Reservations']:
        for instance in reservation['Instances']:
            instance_state_dict[instance['InstanceId']] = instance['State']['Name']

    all_running = all(map(lambda instanceId: instanceId in instance_state_dict.keys() and instance_state_dict[instanceId] == 'running', instance_ids))
    if all_running:
        print('all instances created are now running. proceeding')
        print(instance_state_dict)
        break
    else:
        print("still waiting for instances to get in a running state:")
        print(instance_state_dict)
        time.sleep(5.0)

time.sleep(30.0) # wait a bit longer to make sure ssm knows the instance is in the correct state
ssm_client = boto3.client('ssm')
print(instance_ids)
# have them all grab the script and run it
response = ssm_client.send_command(
    InstanceIds=instance_ids,
    DocumentName="AWS-RunShellScript",
    Parameters={
        'commands': [
            'aws s3 cp '+script_url+' /home/ec2-user',
            'chmod +x /home/ec2-user/{0}'.format(script_name),
            'sh /home/ec2-user/{0} {1} {2} {3} {4} {5} {6} {7}'
                .format(
                    script_name, 
                    upload_prefix, 
                    blender_project_archive_s3_path,
                    blender_project_archive_filename,
                    blender_project_path,
                    blender_archive_s3_path,
                    blender_archive_filename,
                    blender_executable_path
                )
        ]
    }
)

command_id = response['Command']['CommandId']
print("SSM command executed. Command ID: "+command_id)

# poll the command until it is done, then terminate all of the instances
original_instances = set(instance_ids)
instances_terminated = set()

time.sleep(20.0) # wait a bit before calling get_command_invocation
while instances_terminated != original_instances:
    print("Original Instances: " + str(original_instances))
    print("Instances Terminated: " + str(instances_terminated))
    for instance_id in original_instances - instances_terminated:
        command_state = ssm_client.get_command_invocation(
            CommandId=command_id,
            InstanceId=instance_id,
            PluginName='aws:RunShellScript'
        )

        curr_status = command_state['Status']
        if curr_status in ['Success', 'Cancelled', 'TimedOut', 'Failed']:
            # if the command is finished on an instance, terminate
            print(instance_id + " is in status " + curr_status + ", terminating...")
            ec2_client.terminate_instances(InstanceIds = [instance_id])
            instances_terminated.add(instance_id)
        else:
            print(instance_id + " is in state " + curr_status)
    
    time.sleep(5.0)
