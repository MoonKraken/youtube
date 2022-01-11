import { 
  Stack, 
  StackProps, 
  aws_lambda as lambda, 
  aws_iam as iam,
  aws_dynamodb as ddb,
  aws_s3 as s3,
  aws_dynamodb,
} from 'aws-cdk-lib';
import { Construct } from 'constructs';

export class IntroductionToCdkStack extends Stack {
  constructor(scope: Construct, id: string, props?: StackProps) {
    super(scope, id, props);

    const ddbUsersTable = new ddb.Table(this, 'Users', {
      tableName: 'users',
      partitionKey: {
        name: 'uid',
        type: aws_dynamodb.AttributeType.STRING
      }
    });

    const fn = new lambda.Function(this, 'IntroductionToCdkFunction', {
      runtime: lambda.Runtime.PROVIDED_AL2,
      handler: 'provided',
      code: lambda.Code.fromAsset(
        '/Users/kenk/Documents/Code/Repositories/youtube/RustOnAWS/lambda.zip'
      ),
      architecture: lambda.Architecture.ARM_64
    });

    ddbUsersTable.grantWriteData(fn)
  }
}
