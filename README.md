# awesome_wasm

The core idea of this project is to demonstrate a way to replace containers(docker) by implementing Webassembly modules 
which is sandboxed by default and has very small foot print and many more benefits

Webassembly was originally deisgned for web browsers, but in recent days after realizing its possible potential on server-sude side many are experimenting WASM in server-side
this idea is also one such experiment to demonstrate the ability of WASM on server-side mainly on the cloud

The demo is planned to be provided using any cloud solutions(preferably Google Cloud Platform, ofcourse in our personal account)
Through out the project we may need a lot of dependencies importantly Kubectl with proper config to kubernetes engine and Krustlet

#### the following functionalities will be covered
1. Compile source to .wasm file with credentials sigin
2. Move the created wasm module to Container Registry in cloud (ex: GCR)
3. Create a Kubernetes Cluster (ex:GKE)
4. Add a Node to the cluster that can resolve the WASM Module (ex:GCE)
5. Create kube POD using the OCI image in step 2
Make some CURL reqs to verify the concept is indeed a reality


### Some friendly notes:
#### RUST is our language of choice
1. At this moment a minimal primorial factor app is developed to demonstrate the concept(packaging & deployment are the hero of this project not the app itself :) )
2. Its highly advisable to use LINUX or Mac for this type of work, if windows is preferred for some reason, kindly use WSL
3. OneIT does not have Rust lang avaialble in their repository so we have planned to use or personal computes or cloud computes


### Our Inspiration:
1. It all started with a tweet from the founder of Docker himself stating >"If WASM+WASI existed in 2008, we wouldn't have needed to created Docker. 
		That's how important it is. Webassembly on the server is the future of computing. A standardized system interface was the missing link.
		Let's hope WASI is up to the task!"

2. Some inspiring open source work from bytecodealliance and Deislabs

Happy hacking !!


with :>
***bits N' bytes***
