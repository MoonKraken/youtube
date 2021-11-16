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
    't2.micro'
]
instance_ids = []

for instance_type in instances_to_profile:
    # scp the shell script to run
    # create a new EC2 instance
    instance_ids.append(ec2.create_instances(
        ImageId='ami-00be885d550dcee43',
        MinCount=1,
        MaxCount=1,
        InstanceType=instance_type,
        IamInstanceProfile={
            'Name': 'AmazonSSMRoleForInstancesQuickSetup'
        }
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

    all_running = all(map(lambda instanceId: instance_state_dict[instanceId] == 'running', instance_ids))
    if all_running:
        print('all instances created are now running. proceeding')
        print(instance_state_dict)
        break
    else:
        print("still waiting for instances to get in a running state:")
        print(instance_state_dict)
        time.sleep(5.0)

time.sleep(10.0) # wait a bit longer to make sure ssm knows the instance is in the correct state
ssm_client = boto3.client('ssm')
print(instance_ids)
# have them all grab the script and run it
response = ssm_client.send_command(
    InstanceIds=instance_ids,
    DocumentName="AWS-RunShellScript",
    Parameters={
        'commands': [
            'aws s3 cp '+script_url+' ~/',
            'chmod +x ~/' + script_name,
            'sh ~/' + script_name + ' ' + upload_prefix 
        ]
    }
)

command_id = response['Command']['CommandId']
print(command_id)