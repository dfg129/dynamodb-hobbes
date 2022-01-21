import { Stack, StackProps, RemovalPolicy } from 'aws-cdk-lib';
import { Construct } from 'constructs';
import { Vpc, SubnetType  } from 'aws-cdk-lib/aws-ec2';
import { Policy, PolicyStatement } from 'aws-cdk-lib/aws-iam';
import * as lambda from 'aws-cdk-lib/aws-lambda';
import { RetentionDays } from 'aws-cdk-lib/aws-logs';
import * as s3 from 'aws-cdk-lib/aws-s3';
import * as s3n from 'aws-cdk-lib/aws-s3-notifications';
import * as eventsources from 'aws-cdk-lib/aws-lambda-event-sources';
import * as path from 'path';
import * as app from 'aws-cdk-lib/aws-appconfig'


export class DynamodbStack extends Stack {
  constructor(scope: Construct, id: string, props?: StackProps) {
    super(scope, id, props);

    let docker_dir = path.join(__dirname, '../../srvls');

    let vpc = Vpc.fromLookup(this, "cafe-vpc", { vpcName: 'cafe-vpc' });

    const dynamodb_create_policy = new PolicyStatement({
        actions: [
                "dynamodb:ListTables",
                "dynamodb:DescribeTable",
                "dynamodb:CreateTable"
        ],
        resources: ["*"]
    });

    const dynamodb_read_policy = new PolicyStatement({
        actions: [
                "dynamodb:BatchGetItem",
				"dynamodb:Get*",
        ],
        resources: ["*"]
    });

    const dynamodb_write_policy = new PolicyStatement({
        actions: [
				"dynamodb:BatchWrite*",
				"dynamodb:PutItem",
        ],
        resources: ["*"]
    });

let fn = new lambda.DockerImageFunction(this, 'dynamodb-create-table', {
        description:'Create a dynamodb table',
         code: lambda.DockerImageCode.fromImageAsset( path.join(docker_dir, '/create') ),
        architecture: lambda.Architecture.ARM_64,
        environment: {
            RUST_BACKTRACE: '1',
        } ,
        vpc: vpc,
        vpcSubnets: { subnetType: SubnetType.PRIVATE_WITH_NAT },
        logRetention: RetentionDays.ONE_WEEK,
    });

    fn.role?.attachInlinePolicy(
        new Policy(this, 'dynamodb-create-policy', {
            statements: [dynamodb_create_policy],
        }),
    );

/*
    let fn2 = new lambda.DockerImageFunction(this, 'dynamodb-read-table', {
        description: 'Read from a dynamodb table',
        code: lambda.DockerImageCode.fromImageAsset(  path.join(docker_dir, '/read') ),
        architecture: lambda.Architecture.ARM_64,
        environment: {
            RUST_BACKTRACE: '1',
        } ,
        vpc: vpc,
        vpcSubnets: { subnetType: SubnetType.PRIVATE_WITH_NAT },
        logRetention: RetentionDays.ONE_WEEK,
    });

    fn2.role?.attachInlinePolicy(
        new Policy(this, 'dynamodb-read-policy', {
            statements: [dynamodb_read_policy],
        }),
    );


    let fn3 = new lambda.DockerImageFunction(this, 'dynamodb-write-table', {
        description: 'Write into a dynamodb table',
        code: lambda.DockerImageCode.fromImageAsset( path.join(docker_dir, '/write' )),
        architecture: lambda.Architecture.ARM_64,
        environment: {
            RUST_BACKTRACE: '1',
        } ,
        vpc: vpc,
        vpcSubnets: { subnetType: SubnetType.PRIVATE_WITH_NAT },
        logRetention: RetentionDays.ONE_WEEK,
    });

    fn3.role?.attachInlinePolicy(
        new Policy(this, 'dynamodb-write-policy', {
            statements: [dynamodb-write-policy],
        }),
    );
 */
  }
}
