public class MethodInvocationReturningString {
    public static void main(String[] args) {
        System.out.println(aMethod());
    }

    private static String aMethod() {
        return "a string returned from a method";
    }
}