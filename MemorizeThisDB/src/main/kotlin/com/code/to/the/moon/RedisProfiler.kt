package com.code.to.the.moon

import com.code.to.the.moon.model.UserData
import redis.clients.jedis.*
import java.util.*
import kotlin.system.measureNanoTime

class RedisProfiler(
    hostname: String,
    port: Int,
    ssl: Boolean
) : Profiler {

    private val jedis = JedisCluster(
        Collections.singleton(HostAndPort(hostname, port)),
        DefaultJedisClientConfig.builder().ssl(ssl).build(),
        5,
        JedisPoolConfig()
    )

    override fun profileDataPopulation(genericRecords: List<UserData>): Float {
        var totalWriteTime = 0L
        genericRecords.forEach { userData ->
            val idString = userData.id.toString()
            userData.fieldMap().forEach { (field, value) ->
                totalWriteTime += measureNanoTime {
                    jedis.hset(idString, field, value)
                }
            }
        }

        return totalWriteTime / genericRecords.size.toFloat()
    }

    override fun profileReads(ids: Set<Int>): Pair<Float, Float> {
        var firstReadTotal = 0L
        var secondReadTotal = 0L
        ids.map { it.toString() }.forEach { id ->
            val firstRead = measureNanoTime {
                jedis.hgetAll(id)
            }

            firstReadTotal += firstRead

            val secondRead = measureNanoTime {
                jedis.hgetAll(id)
            }

            secondReadTotal += secondRead
        }

        return Pair(firstReadTotal.toFloat() / ids.size.toFloat(), secondReadTotal.toFloat() / ids.size.toFloat())
    }

    override fun profileCleanup(ids: Set<Int>): Float {
        var totalDeleteTime = 0L
        ids.forEach {
            totalDeleteTime += measureNanoTime {
                jedis.del(it.toString())
            }

        }

        return totalDeleteTime / ids.size.toFloat()
    }
}