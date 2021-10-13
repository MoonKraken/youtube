import software.amazon.awssdk.regions.Region
import java.time.Duration
import java.time.LocalDateTime
import java.util.*

object IfYouHaveTime {

    private val rng = Random(42)
    private const val MAX_DEVIATION = 10
    private val REGION = Region.US_WEST_2

    @JvmStatic
    fun main(args: Array<String>) {
        val startingStockPrices = mapOf(
            "FB" to 200.0,
            "AMZN" to 3500.0,
            "AAPL" to 300.0,
            "NLFX" to 425.0,
            "GOOG" to 10.0
        )

        val timestamps = generateTimestamps(
            LocalDateTime.parse("2021-09-28T00:00:00"),
            LocalDateTime.parse("2021-10-05T00:00:00"),
            1
        )

        println("Number of timestamps generated: ${timestamps.size}")

        // TICKER -> (DATETIME/PRICE pairs)
        val genericRecords: Map<String, Map<LocalDateTime, Double>> = startingStockPrices.map {
            val ticker = it.key
            val startingPrice = it.value
            val priceMap = HashMap<LocalDateTime, Double>()
            var lastPrice = startingPrice

            // Let the stock price go on a random walk, but never going below 1.0
            timestamps.forEach { currentTimestamp ->
                val newPrice = (lastPrice + rng.nextDouble() * MAX_DEVIATION - (0.5 * MAX_DEVIATION)).coerceAtLeast(1.0)
                priceMap[currentTimestamp] = newPrice
                lastPrice = newPrice
            }

            ticker to priceMap
        }.toMap()

        val veryStart = System.currentTimeMillis()
        // TimestreamWriter("StockPrices", "FANG", REGION).writeToTimestream(genericRecords)
        val timeStreamComplete = System.currentTimeMillis()
        // CloudWatchWriter("FANG", "StockPrices", REGION).publishToCloudWatch(genericRecords)
        val cloudWatchWriterComplete = System.currentTimeMillis()
        DynamoDBWriter("ifyouhavetime", REGION).writeToDynamoDb(genericRecords)
        val dynamoDbWriterComplete = System.currentTimeMillis()
        //RDSWriter("ifyouhavetime", "StockPrices", REGION).writeToRds(genericRecords)
        val rdsWriteComplete = System.currentTimeMillis()

        println("Timestream write time: ${timeStreamComplete - veryStart}")
        println("Cloudwatch write time: ${cloudWatchWriterComplete - timeStreamComplete}")
        println("DynamoDB write time: ${dynamoDbWriterComplete - cloudWatchWriterComplete}")
        println("RDS write time: ${rdsWriteComplete - dynamoDbWriterComplete}")
    }

    private fun generateTimestamps(
        start: LocalDateTime,
        end: LocalDateTime,
        intervalHours: Int
    ): List<LocalDateTime> {
        val durationBetween = Duration.between(start, end)
        val numTimestamps = durationBetween.toHours() / intervalHours
        return (0 until numTimestamps).map {
            start.plusHours(it * intervalHours)
        }
    }
}