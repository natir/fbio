# Friday's bioinformatics optimizations

Just a repository to store sequence bioinformatics microbenchmark and script to produce graph.

## Python requirement

- plotly

## cano_iter

Run benchmark:
```
cargo bench -- cano_iter
```

Open `target/criterion/iter_cano/report/index.html` to see criterion report.

## nuc2bit

Run benchmark:
```
cargo bench -- nuc2bit
```

Open `target/criterion/nuc2bit/report/index.html` to see criterion report.

Or you can generate ploty graph:
```
// To compare median
python -c "import fbio; fbio.nuc2bit.median_error()"

// To compare average
python -c "import fbio; fbio.nuc2bit.average_error()"
```
