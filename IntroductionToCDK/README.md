# Code To the Moon Notes

Bootstrap Policies required
```
IAMFullAccess
AmazonEC2ContainerRegistryFullAccess
AmazonS3FullAccess
AWSCodeDeployFullAccess
AmazonSSMFullAccess
AWSCloudFormationFullAccess
AWSLambda_FullAccess
```

After bootstrapping, you'll need your role to be able to assume a certain role that the bootstrapping process created, which in my case was arn:aws:iam::765643058521:role/cdk-hnb659fds-deploy-role-765643058521-us-west-2. I had to manually add sts::AssumeRole * as an inline policy to my user, as there didn't seem to be any prebuilt AWS policies with this permission.

# Welcome to your CDK TypeScript project!

This is a blank project for TypeScript development with CDK.

The `cdk.json` file tells the CDK Toolkit how to execute your app.

## Useful commands

 * `npm run build`   compile typescript to js
 * `npm run watch`   watch for changes and compile
 * `npm run test`    perform the jest unit tests
 * `cdk deploy`      deploy this stack to your default AWS account/region
 * `cdk diff`        compare deployed stack with current state
 * `cdk synth`       emits the synthesized CloudFormation template