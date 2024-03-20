// import('./factorial_calculator').then(module => {
//     window.calculateFactorial = () => {
//         const input = document.getElementById('numberInput').value;
//         const answer = module.factorial(parseInt(input));
//         document.getElementById('result').innerText = answer;
//         console.log("Test");
//     };
// }).catch(console.error);


import('./factorial_calculator').then(module => {
    window.calculateFactorial = () => {
        const input = document.getElementById('numberInput').value;
        const result = module.factorial(parseInt(input));
        document.getElementById('result').innerText = result;
        console.log("Test");
    };
    document.getElementById('calculateButton').onclick = calculateFactorial;
}).catch(console.error);
