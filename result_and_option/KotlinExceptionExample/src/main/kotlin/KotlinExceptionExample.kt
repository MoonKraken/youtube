class SummationException: Exception("Issue summing array")

fun toInt(s: String): Int? {
    return Integer.parseInt(s)
}

fun sumStrVec(strings: Array<String>): String {
    var accum = 0
    for (s: String in strings) {
       accum += try {
           toInt(s)
       } catch (e: Exception) {
           throw SummationException()
       } ?: 0
    }

    return accum.toString()
}
fun main(args: Array<String>) {
    val v = arrayOf("3", "4")
    val total = sumStrVec(v)
    println(total)

    val v2 = arrayOf("3", "abc")
    val total2 = sumStrVec(v2)
    println(total2)
}