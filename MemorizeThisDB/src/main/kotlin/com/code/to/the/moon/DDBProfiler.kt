package com.code.to.the.moon

import com.code.to.the.moon.model.UserData
import software.amazon.awssdk.core.SdkBytes
import software.amazon.awssdk.regions.Region
import software.amazon.awssdk.services.dynamodb.DynamoDbClient
import software.amazon.awssdk.services.dynamodb.model.*
import software.amazon.dax.ClusterDaxClient
import software.amazon.dax.Configuration
import java.lang.Math.ceil
import java.lang.Thread.sleep
import kotlin.system.measureTimeMillis

class DDBProfiler(
    region: Region,
    private val tableName: String,
    daxHostname: String? = null
) : Profiler {
    companion object {
        const val BATCH_SIZE = 25
    }

    private val ddbClient: DynamoDbClient = daxHostname?.let {
        ClusterDaxClient.builder()
            .overrideConfiguration(
                Configuration.builder()
                    .url(daxHostname)
                    .region(region)
                    .build()
            ).build()
    } ?: run {
        DynamoDbClient.builder()
            .region(region)
            .build()
    }

    private fun genericRecordsToDdbWriteRequest(
        genericRecords: List<UserData>
    ): List<WriteRequest> {
        return genericRecords.map { genericRecord ->
            val itemValues = mutableMapOf<String, AttributeValue>(
                "id" to AttributeValue.builder().s(genericRecord.id.toString()).build(),
                "firstName" to AttributeValue.builder().s(genericRecord.firstName).build(),
                "lastName" to AttributeValue.builder().s(genericRecord.lastName).build(),
                "age" to AttributeValue.builder().n(genericRecord.age.toString()).build(),
                "dietaryRestrictions" to AttributeValue.builder().s(genericRecord.dietaryRestrictions.toString()).build(),
                "tshirtSize" to AttributeValue.builder().s(genericRecord.tshirtSize.toString()).build(),
                "profilePicture" to AttributeValue.builder().bs(SdkBytes.fromByteArray(genericRecord.profilePicture)).build()
            )

            WriteRequest.builder()
                .putRequest(
                    PutRequest.builder()
                        .item(itemValues)
                        .build()
                ).build()
        }
    }

    override fun profileDataPopulation(genericRecords: List<UserData>): Float {
        val writeRequests = genericRecordsToDdbWriteRequest(genericRecords)

        return batchWriteOperation(writeRequests)
    }

    // first return value is first read, second is for the read immediately after
    override fun profileReads(ids: Set<Int>): Pair<Float, Float> {
        var totalTimeFirstRead: Long = 0
        var totalTimeSecondRead: Long = 0
        ids.forEach {
            val getItemRequest = GetItemRequest.builder()
                .tableName(tableName)
                .key(mapOf("id" to AttributeValue.builder().s(it.toString()).build()))
                .build()

            val res: GetItemResponse
            totalTimeFirstRead += measureTimeMillis {
                res = ddbClient.getItem(getItemRequest)
            }

            val res2: GetItemResponse
            totalTimeSecondRead += measureTimeMillis {
                res2 = ddbClient.getItem(getItemRequest)
            }

            if (it % 10 == 0) {
                println(res.item())
                println(res2.item())
            }
        }

        return Pair(
            totalTimeFirstRead.toFloat() / ids.size.toFloat(),
            totalTimeSecondRead.toFloat() / ids.size.toFloat()
        )
    }

    private fun batchWriteOperation(writeRequests: List<WriteRequest>): Float {
        var totalTimeForAllOperations: Long = 0
        (writeRequests.indices step BATCH_SIZE).forEach {
            val batchRequest = BatchWriteItemRequest.builder()
                .requestItems(
                    mapOf(tableName to writeRequests.subList(it, (it + BATCH_SIZE).coerceAtMost(writeRequests.size - 1)))
                )
                .build()

            try {
                //nap a bit so we dont exceed provisioned capacity
                sleep(100)
                val writeTime = measureTimeMillis {
                    ddbClient.batchWriteItem(batchRequest)
                }
                println("Batch write took ${writeTime}ms")
                totalTimeForAllOperations += writeTime
            } catch (e: Exception) {
                println("Issue writing to DDB")
                e.printStackTrace()
            }
        }

        return totalTimeForAllOperations.toFloat() / kotlin.math.ceil(writeRequests.size.toDouble() / BATCH_SIZE).toFloat()
    }

    override fun profileCleanup(ids: Set<Int>): Float {
        val deleteRequests = ids.map {
            val deleteItemRequest: DeleteRequest = DeleteRequest.builder()
                .key(mapOf("id" to AttributeValue.builder().s(it.toString()).build()))
                .build()

            WriteRequest.builder()
                .deleteRequest(deleteItemRequest)
                .build()
        }

        return batchWriteOperation(deleteRequests)
    }
}