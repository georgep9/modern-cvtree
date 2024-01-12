### Overview of CVTree

CVTree is a C++ program that generates a dissimilarity matrix between organisms based on their genomes. The dissimilarity matrix represents a phylogenetic tree that infers the evolutionary relationships between organisms [1]. Compared to other methods in comparative genomics (how organisms are related at the genetic level), CVTree constructs a phylogenetic tree without sequence alignment by using the composition vector (CV) method [2][3]. 

[1] https://www.ebi.ac.uk/training/online/courses/introduction-to-phylogenetics/what-is-phylogenetics/ 
[2] https://opendata.pku.edu.cn/dataset.xhtml?persistentId=doi:10.18170/DVN/XPECRU&language=en 
[3] https://www.nature.com/scitable/knowledge/library/comparative-genomics-13239404/ 

### Logic Flow

![process flow](https://raw.githubusercontent.com/georgep9/modern-cvtree/master/logic-flow.png)

### Pre-existing undergraduate project [QUT]

Original version
```bash
g++ -g original.cpp -o original -fopenmp
./original
```
Parallelized
```bash
g++ -g parallel.cpp -o parallel -fopenmp
./parallel
```

links to https://github.com/georgep9/parallel-cvtree
