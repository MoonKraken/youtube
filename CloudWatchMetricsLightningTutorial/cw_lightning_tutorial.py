import boto3

def lambda_handler(event, context):
    cloudwatch = boto3.client('cloudwatch')

    country = event['country']
    color = event['color']
    num_purchased = event['num_purchased']

    cloudwatch.put_metric_data(
        MetricData=[
            {
                'MetricName': 'Widgets Purchased',
                'Dimensions': [
                    {
                        'Name': 'Country',
                        'Value': country
                    },
                    {
                        'Name': 'Color',
                        'Value': color
                    }
                ],
                'Unit': 'None',
                'Value': float(num_purchased)
            }
        ],
        Namespace='Tests'
    )

    return {
        'statusCode': 200,
        'body': 'metric published'
    }