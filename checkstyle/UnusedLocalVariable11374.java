// https://github.com/checkstyle/checkstyle/issues/11374
public class UnusedLocalVariable11374 {

    static class m {
        static class h {
            public int a = 12;
        }
    }

    static class j {
        static void method() {
            int a = 1000; // unexpected violation
            m.h obj = new m.h() { // object of Test.j.m.h and not of Test.m.h
                @Override
                void method() {
                    System.out.println("a = " + a); // local var a
                }
            };
            obj.method();
        }

        static class m {
            static class h {
                void method() {
                }
            }
        }
    }

    public static void main(String[] args) {
        j.method();
    }
}
