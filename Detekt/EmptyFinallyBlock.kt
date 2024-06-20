fun main() {
    try {
        val result = 10 / 2
        println("Result: $result")
    } catch (e: ArithmeticException) {
        println("An error occurred: ${e.message}")
    } finally {
    }
}
