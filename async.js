
async function asyncFunc() {

    return new Promise((resolve, reject) => {
        
        
        resolve("Hello from async")

    }).then(s => console.log(s))
    .catch(err => console.log(err));
}

await asyncFunc();

console.log("Hello")