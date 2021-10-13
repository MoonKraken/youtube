import software.amazon.awssdk.core.client.config.ClientOverrideConfiguration
import software.amazon.awssdk.core.retry.RetryPolicy
import software.amazon.awssdk.http.apache.ApacheHttpClient
import software.amazon.awssdk.regions.Region
import software.amazon.awssdk.services.timestreamwrite.TimestreamWriteClient
import software.amazon.awssdk.services.timestreamwrite.model.*
import java.time.Duration
import java.time.LocalDateTime
import java.time.ZoneOffset

class TimestreamWriter(
    private val databaseName: String,
    private val tableName: String,
    private val region: Region
) {
    companion object {
        const val MAX_BATCH_WRITE_COUNT = 100
    }

    private val writeClient: TimestreamWriteClient

    // from https://docs.aws.amazon.com/timestream/latest/developerguide/code-samples.write-client.html
    init {
        val httpClientBuilder: ApacheHttpClient.Builder = ApacheHttpClient.builder()
        httpClientBuilder.maxConnections(5000)
        val retryPolicy: RetryPolicy.Builder = RetryPolicy.builder()
        retryPolicy.numRetries(10)
        val overrideConfig: ClientOverrideConfiguration.Builder = ClientOverrideConfiguration.builder()
        overrideConfig.apiCallAttemptTimeout(Duration.ofSeconds(20))
        overrideConfig.retryPolicy(retryPolicy.build())
        writeClient = TimestreamWriteClient.builder()
            .httpClientBuilder(httpClientBuilder)
            .overrideConfiguration(overrideConfig.build())
            .region(region)
            .build()
    }

    private fun generateTimestreamRecords(
        genericRecords: Map<String, Map<LocalDateTime, Double>>
    ): List<Record> {
        return genericRecords.flatMap { genericRecord ->
            val ticker = genericRecord.key
            val priceMap = genericRecord.value
            priceMap.map {
                val timestamp = it.key
                val price = it.value
                Record.builder()
                    .dimensions(
                        listOf(Dimension.builder().name("ticker").value(ticker).build())
                    )
                    .measureValueType(MeasureValueType.DOUBLE)
                    .measureName("price")
                    .measureValue(price.toString())
                    .time((timestamp.toEpochSecond(ZoneOffset.UTC) * 1000).toString())
                    .build()
            }
        }
    }

    fun writeToTimestream(genericRecords: Map<String, Map<LocalDateTime, Double>>) {
        val records = generateTimestreamRecords(genericRecords)

        try {
            (records.indices step MAX_BATCH_WRITE_COUNT).forEach {
                val writeRecordsRequest: WriteRecordsRequest =
                    WriteRecordsRequest.builder()
                        .databaseName(databaseName)
                        .tableName(tableName)
                        .records(records.subList(it, (it + MAX_BATCH_WRITE_COUNT).coerceAtMost(records.size-1)))
                        .build()

                val writeRecordsResponse: WriteRecordsResponse = writeClient.writeRecords(writeRecordsRequest)
                println("WriteRecords Status: " + writeRecordsResponse.sdkHttpResponse().statusCode())
            }
        } catch (e: RejectedRecordsException) {
            println("RejectedRecords: $e")
            for (rejectedRecord: RejectedRecord in e.rejectedRecords()) {
                println(
                    "Rejected Index " + rejectedRecord.recordIndex() + ": "
                            + rejectedRecord.reason()
                )
            }
            println("Other records were written successfully. ")
        } catch (e: Exception) {
            println("Error: $e")
        }
    }
}