
- Create a package.tar.gz file for the package


- tar --transform='s|^src|xyz|' -czf xyz.tar.gz $(find src -name '*.nk')

- tar -xvf xyz.tar.gz 

xyz/test/cats.nk
xyz/project.nk
