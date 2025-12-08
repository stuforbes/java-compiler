public class MethodInvocationReturningStringArray {
    public static void main(String[] args) {
        System.out.println(aMethod()[0]);
    }

    private static String[] aMethod() {
        return new String[] { "a string array returned from a method" };
    }
}