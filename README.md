# idm

idm (IcarusDownloadManager) is a CLI software client application that has the feature of uploading and downloading songs from the [icarus](https://git.kundeng.us/phoenix/icarus) Music Server. 



### Getting Started

Clone the repo

```BASH
git clone git@git.kundeng.us:phoenix/idm.git
```


Build the project:

```BASH
cd idm
cargo build
```


### Downloading Song

```BASH
idm download -u spacecadet -p stellar40 -h https://icarus.com -ha https://auth.icarus.com -b e8407fc6-edd2-44c1-993f-08dd7324d91a
```

### Uploading Song with metadata

```BASH
idm upload-meta -u spacecadet -p stellar40 -h https://icarus.com -ha https://auth.icarus.com -s /path/of/song.flac -t 1 -m /path/to/metadata/config/collection.json -ca /path/to/cover/art/image.png
```

### Uploading Song with metadata from directory

```BASH
idm upload-meta -u spacecadet -p stellar40 -h https://icarus.com -ha https://auth.icarus.com -smca /path/where/songs/and/metadata/exists/
```

### Retrieving Song in json

```Bash
idm retrieve -u spacecadet -p stellar40 -h https://icarus.com -ha https://auth.icarus.com -rt songs
```

### Deleting Song

```BASH
idm delete -u spacecadet -p stellar40 -h https://icarus.com -ha https://auth.icarus.com -D e8407fc6-edd2-44c1-993f-08dd7324d91a
```


## Contributing

Please read [CONTRIBUTING.md](CONTRIBUTING.md) for details on the code of conduct, and the process for submitting pull requests to the project.



## License

This project is licensed under the MIT License - see the [LICENSE.md](LICENSE.md) file for details
