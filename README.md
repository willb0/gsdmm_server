# GSDMM_server

This is a rust web server I wrote to serve a GSDMM model

GSDMM stands for Gibbs Sampling Dirichlet Mixture Model, a model used to cluster short text documents [link to paper](https://dbgroup.cs.tsinghua.edu.cn/wangjy/papers/KDD14-GSDMM.pdf)
- /validate_body POST
  - Returns the json body if valid, or an error. schema below
    ```rust
    pub documents: Vec<Vec<String>>,
    pub vocab: Vec<String>,
    pub max_clusters: usize,
    pub alpha: f64,
    pub beta: f64,
    ```
- /model_endpoint POST
  - Returns the cluster labels per document, or an error. schema is same as above

### To run
this is built for arm64 (mac m1), so you might have to tweak the cargo target in dockerfile to compile on x86
```
git clone git@github.com:willb0/gsdmm_server.git
docker build -t gsdmm_server .
docker run -it
docker run --rm -it -p 8080:8080  gsdmm_server
```