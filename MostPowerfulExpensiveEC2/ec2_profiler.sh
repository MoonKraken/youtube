#!/bin/sh

# $1 should be the s3 prefix we upload results to
export INSTANCE_TYPE=$(ec2-metadata --instance-type | awk '{print $2}')
echo $INSTANCE_TYPE > instance_type.txt
aws s3 cp ./instance_type.txt $1/$INSTANCE_TYPE/