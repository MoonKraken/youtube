import software.amazon.awssdk.core.exception.SdkClientException
import software.amazon.awssdk.regions.Region
import software.amazon.awssdk.services.cloudwatch.CloudWatchClient
import software.amazon.awssdk.services.cloudwatch.model.*
import java.time.LocalDateTime
import java.time.ZoneOffset
import kotlin.system.exitProcess

class CloudWatchWriter(
    private val metricName: String,
    private val namespace: String,
    region: Region
) {
    companion object {
        const val BATCH_SIZE = 20
    }
    private val cloudWatchClient: CloudWatchClient = CloudWatchClient.builder()
        .region(region)
        .build();

    private fun generateCloudWatchDatums(
        genericRecords: Map<String, Map<LocalDateTime, Double>>
    ): List<MetricDatum> {
        return genericRecords.flatMap { genericRecord ->
            val ticker = genericRecord.key
            val priceMap = genericRecord.value
            priceMap.map {
                val timestamp = it.key
                val price = it.value

                val dimension = Dimension.builder()
                    .name("ticker")
                    .value(ticker)
                    .build()

                MetricDatum.builder()
                    .metricName(metricName)
                    .unit(StandardUnit.NONE)
                    .value(price)
                    .timestamp(timestamp.toInstant(ZoneOffset.UTC))
                    .dimensions(dimension)
                    .build()
            }
        }
    }

    fun publishToCloudWatch(genericRecords: Map<String, Map<LocalDateTime, Double>>) {
        val datums = generateCloudWatchDatums(genericRecords)

        try {
            (datums.indices step BATCH_SIZE).forEach {
                val request = PutMetricDataRequest.builder()
                    .namespace(namespace)
                    .metricData(datums.subList(it, (it + BATCH_SIZE).coerceAtMost(datums.size - 1)))
                    .build();

                cloudWatchClient.putMetricData(request);
            }
        } catch (e: SdkClientException) {
            println(e.cause)
            exitProcess(1)
        }
    }
}