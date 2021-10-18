package com.code.to.the.moon

import com.code.to.the.moon.model.UserData

interface Profiler {
    fun profileDataPopulation(genericRecords: List<UserData>): Float
    fun profileReads(ids: Set<Int>): Pair<Float, Float>
    fun profileCleanup(ids: Set<Int>): Float
}