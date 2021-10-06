import software.amazon.awssdk.regions.Region
import software.amazon.awssdk.services.rdsdata.RdsDataClient
import software.amazon.awssdk.services.rdsdata.model.BatchExecuteStatementRequest
import software.amazon.awssdk.services.rdsdata.model.Field
import software.amazon.awssdk.services.rdsdata.model.SqlParameter
import java.time.LocalDateTime
import java.time.format.DateTimeFormatter

class RDSWriter(
    region: Region,
    private val databaseName: String,
    private val tableName: String
) {
    private val rdsClient: RdsDataClient = RdsDataClient.builder().region(region).build()

    private fun generateRdsRequest(genericRecords: Map<String, Map<LocalDateTime, Double>>): List<List<SqlParameter>> {
        return genericRecords.flatMap { genericRecord ->
            val ticker = genericRecord.key
            val priceMap = genericRecord.value
            priceMap.map {
                val timestamp = it.key
                val price = it.value

                val tickerParameter = SqlParameter.builder().name("ticker").value(
                    Field.builder().stringValue(ticker).build()).build()
                val timestampParameter = SqlParameter.builder()
                    .name("timestamp")
                    .value(Field.builder()
                        .stringValue(
                            timestamp.format(
                                DateTimeFormatter.ISO_DATE_TIME)).build()
                    )
                    .build()
                val priceParameter = SqlParameter.builder()
                    .name("price")
                    .value(
                        Field.builder()
                            .doubleValue(price)
                            .build())
                    .build()

                listOf(tickerParameter, timestampParameter, priceParameter)
            }
        }
    }

    fun writeToRds(genericRecords: Map<String, Map<LocalDateTime, Double>>) {
        val parameterSets = generateRdsRequest(genericRecords)

        try {
            parameterSets.indices.forEach {
                val executeStatement = BatchExecuteStatementRequest.builder()
                    .database(databaseName)
                    .sql("INSERT INTO $tableName values (:ticker, :timestamp, :price")
                    .parameterSets(parameterSets[it])
                    .build()

                rdsClient.batchExecuteStatement(executeStatement)
            }
        } catch (e: Exception) {

        }
    }
}