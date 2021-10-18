package com.code.to.the.moon

import TestContext
import com.amazonaws.services.lambda.runtime.Context
import com.amazonaws.services.lambda.runtime.RequestHandler
import com.code.to.the.moon.model.DietaryRestrictions
import com.code.to.the.moon.model.TshirtSize
import com.code.to.the.moon.model.UserData
import software.amazon.awssdk.regions.Region
import java.lang.Thread.sleep
import kotlin.random.Random

class MemorizeThisDB : RequestHandler<Object, String> {
    companion object {
        private const val NUM_USERS = 1000
        private const val NUM_USERS_READ = 100
        private const val NUM_USERS_UPDATE = 100
        private val USER_INDICES = (0 until NUM_USERS)
        private val rng = Random(1)
        private val possibleFirstNames = listOf("Joe", "Karen", "Ken", "Matt", "Justine", "Art", "Jill", "Emily", "Frank", "Ashley", "Ava", "Bob", "Mike", "Harry", "Ola", "Rich")
        private val possibleLastNames = listOf("Smith", "Windsor", "Jones", "Stone", "Rivers", "Mahomes", "Brady", "Warner", "Wilson", "Brackeen", "Atkins", "Williams", "Ishi", "Froning")

        fun generateMockData(numUsers: Int): List<UserData> {
            return (0 until numUsers).map {
                UserData(
                    it,
                    possibleFirstNames.random(rng),
                    possibleLastNames.random(rng),
                    rng.nextInt(100),
                    DietaryRestrictions.values().random(rng),
                    TshirtSize.values().random(rng),
                    rng.nextBytes(32 * 32)
                )
            }
        }

        @JvmStatic
        fun main(args: Array<String>) {
            MemorizeThisDB().handleRequest(Object(), TestContext())
        }
    }

    override fun handleRequest(input: Object, conext: Context): String {

        // Generate mock data in generic format
        val initialData = generateMockData(NUM_USERS)
        val idsToRead = USER_INDICES.shuffled(rng).subList(0, NUM_USERS_READ).toSet()
        val dataUpdates = generateMockData(NUM_USERS_UPDATE)
        val idsToUpdate = USER_INDICES.shuffled(rng).subList(0, NUM_USERS_UPDATE)

        dataUpdates.forEachIndexed { index, userData ->
            userData.id = idsToUpdate[index]
        }

        // Test DDB in isolation
        profileDDBBasedSystem(
            "DDB-only",
            Region.US_WEST_2,
            "users",
            initialData,
            idsToRead,
            dataUpdates
        )


        // Test DDB with DAX
        profileDDBBasedSystem(
            "DDB with DAX",
            Region.US_WEST_2,
            "users",
            initialData,
            idsToRead,
            dataUpdates,
            "daxs://memorize-this-db.umlvfx.dax-clusters.us-west-2.amazonaws.com"
        )


        // Test ElastiCache
        profileRedisBasedSystem(
            "ElastiCache",
            "memorize-this-db.umlvfx.clustercfg.usw2.cache.amazonaws.com",
            6379,
            false,
            initialData,
            idsToRead,
            dataUpdates
        )

        // Test MemoryDB for Redis
        profileRedisBasedSystem(
            "MemoryDB",
            "clustercfg.memorize-this-db.umlvfx.memorydb.us-west-2.amazonaws.com",
            6379,
            true,
            initialData,
            idsToRead,
            dataUpdates
        )
         */
        return "Done"
    }

    private fun profileDDBBasedSystem(systemName: String,
                                      region: Region,
                                    tableName: String,
                                    initialData: List<UserData>,
                                    idsToRead:Set<Int>,
                                    dataUpdates:List<UserData>,
                                  daxHostname: String? = null
    ) {
        val ddbProfiler = DDBProfiler(region,tableName, daxHostname)
        val batchCreateAvgLatency = ddbProfiler.profileDataPopulation(initialData)
        println("$systemName batch create avg: $batchCreateAvgLatency")
        println("2 second nap...")
        sleep(2000)

        val (firstReadAvg, secondReadAvg) = ddbProfiler.profileReads(idsToRead)
        println("$systemName first read avg: $firstReadAvg")
        println("$systemName second read avg: $secondReadAvg")

        println("2 second nap...")
        sleep(2000)

        val updateAvgLatency = ddbProfiler.profileDataPopulation(dataUpdates)
        println("$systemName update avg: $updateAvgLatency")

        println("2 second nap...")
        sleep(2000)

        val deleteBatchAvg = ddbProfiler.profileCleanup(USER_INDICES.toSet())
        println("$systemName delete avg: $deleteBatchAvg")
    }

    private fun profileRedisBasedSystem(systemName:String,
                                        hostname: String,
                                        port: Int,
                                        ssl: Boolean,
                                        initialData: List<UserData>,
                                        idsToRead:Set<Int>,
                                        dataUpdates:List<UserData>
    ) {
        val elastiCacheProfiler = RedisProfiler(hostname, port, ssl)

        val batchCreateAvgLatencyElastiCache = elastiCacheProfiler.profileDataPopulation(initialData)
        println("$systemName batch create avg: $batchCreateAvgLatencyElastiCache ns")

        val (firstReadAvgEC, secondReadAvgEC) = elastiCacheProfiler.profileReads(idsToRead)
        println("$systemName first read avg: $firstReadAvgEC ns")
        println("$systemName second read avg: $secondReadAvgEC ns")

        val updateAvgLatencyEC = elastiCacheProfiler.profileDataPopulation(dataUpdates)
        println("$systemName update avg: $updateAvgLatencyEC ns")

        val deleteBatchAvgEC = elastiCacheProfiler.profileCleanup(USER_INDICES.toSet())
        println("$systemName delete avg: $deleteBatchAvgEC ns")
    }
}