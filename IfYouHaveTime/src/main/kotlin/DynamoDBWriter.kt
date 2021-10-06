import software.amazon.awssdk.regions.Region
import software.amazon.awssdk.services.dynamodb.DynamoDbClient
import software.amazon.awssdk.services.dynamodb.model.*
import software.amazon.awssdk.services.timestreamwrite.model.MeasureValueType
import java.time.LocalDateTime
import java.time.ZoneOffset
import java.time.format.DateTimeFormatter


class DynamoDBWriter(
    private val tableName: String,
    region: Region
) {
    companion object {
        const val BATCH_SIZE = 25
    }
    private val ddbClient: DynamoDbClient = DynamoDbClient.builder()
        .region(region)
        .build()

    private fun genericRecordsToDdbWriteRequest(
        genericRecords: Map<String, Map<LocalDateTime, Double>>
    ): List<WriteRequest> {
        return genericRecords.flatMap { genericRecord ->
            val ticker = genericRecord.key
            val priceMap = genericRecord.value
            priceMap.map {
                val timestamp = it.key
                val price = it.value
                val itemValues = mutableMapOf<String, AttributeValue>(
                    "ticker" to AttributeValue.builder().s(ticker).build(),
                    "timestamp" to AttributeValue.builder().s(timestamp.format(DateTimeFormatter.ISO_DATE_TIME)).build(),
                    "price" to AttributeValue.builder().n(price.toString()).build()
                )

                WriteRequest.builder()
                    .putRequest(
                        PutRequest.builder()
                            .item(itemValues)
                        .build()
                    ).build()
            }
        }
    }

    fun writeToDynamoDb(
        genericRecords: Map<String, Map<LocalDateTime, Double>>
    ) {
        val writeRequests = genericRecordsToDdbWriteRequest(genericRecords)

        (writeRequests.indices step BATCH_SIZE).forEach {
            val batchRequest = BatchWriteItemRequest.builder()
                .requestItems(
                    mapOf(tableName to writeRequests.subList(it, (it + BATCH_SIZE).coerceAtMost(writeRequests.size - 1)))
                )
                .build()

            try {
                ddbClient.batchWriteItem(batchRequest)
            } catch (e: Exception) {
                println("Issue writing to DDB")
                e.printStackTrace()
            }
        }
    }
}