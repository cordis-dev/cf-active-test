public class UnusedVariable {
    int a;

    {
        int k = 12; // violation, assigned and updated but never used
        k++;
    }
}
