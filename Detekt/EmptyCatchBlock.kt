fun main() {
    try {
        val result = 10 / 0
        println("Result: $result")
    } catch (e: ArithmeticException) {
    }
}
