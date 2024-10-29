// Algorithm to implement the ripple carry adder 
//(see https://learn.microsoft.com/en-us/qsharp/api/qsharp-lang/microsoft.quantum.unstable.arithmetic/ripplecarrycgincbyle)

namespace Samples {
    open Microsoft.Quantum.Unstable.Arithmetic;

    @EntryPoint()
    operation EstimateAdder() : Unit {
        let bitsize = 128;

        use xs = Qubit[bitsize];
        use ys = Qubit[bitsize];

        RippleCarryCGIncByLE(xs, ys);
    }
}