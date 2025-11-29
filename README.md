# Box with triangular hole

Note: `./aux-data/meshlab.viewstate.txt` can be copied to 
the clipboard and then pasted into meshlab menu
`[Windows -> Paste clipboard to camera settings]`.
This allows you to see the rendering of stl file
from the same view each time. Also, click on the "Wireframe"
![pic](data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAABgAAAAdCAYAAACwuqxLAAAAAXNSR0IB2cksfwAAAARnQU1BAACxjwv8YQUAAAAgY0hSTQAAeiYAAICEAAD6AAAAgOgAAHUwAADqYAAAOpgAABdwnLpRPAAAAAlwSFlzAAALEwAACxMBAJqcGAAAAAd0SU1FB+kLHBc2OCj3pcIAAATHSURBVEjHzZZBaBNdEIC/JG3TrjE1u6XEutZQcojYkhBCbSQtIhURRA8NHvQgCFoP9SaC4EkE8eBFvClEEUUQPAhK1bRYG2INpNXag/SQS6zRJMom2jTpZrv/qav501L/Q+Gf25vZN9/OzJt5z7S0tKSzidKg65vqHzObLA1rKcvlMqqqoqoqlUqFUqlEPp+nUChQLBbJZDIIgsC+ffuQZZmtW7f+PWBubo6RkRGCwSAul4umpiZaWlpoa2vD4XCQTqeJxWIkEgkOHz6M2+1mcXERp9NJKBTC4/FgMpnWBjx//py5uTm2b9/O27dvOXXqFB6Px7CPjo6yvLzM2bNnURQFn8+HJEkMDQ2RyWS4fv06iqJw9+7d3zXQdR1d10mlUly5cgVRFHE6nfh8PiKRCMlkEk3TePz4MblcjuHhYQD279+P2+1G13Xu3btHIpFAlmXsdjurPnVd/x2BKIr09PQwMDBALBZj165dRrpCoRA/f/4kFApx69Yt3r9/j9ls5vXr1wwMDBCPx/H7/Zw5c4bLly+vX4NUKsXOnTvZvXs3Fy9e5ObNm4b+3LlztLa2IooiXV1dyLLMxMQEJ06coFAo0N7eTj6fp1wur39M8/k8i4uLxjocDvPo0SPcbjderxeXy4XdbmdlZQWLxUIgECASidDZ2QnAwsICFotlfYDFYqFYLBprTdOw2Wx1R0/TNCwWC36/n1gsxrFjxwDI5XJs27atFvBnQTo7O0mn08b6xYsXXLt2jXg8jqqqhr5arWI2m0kmkzQ2NvLr1y8D0NbWVlPkmghkWebz589Gs3379o2uri7y+TyJRKImArPZzNjYGBcuXGBychKAUqmEIAjrp8jpdPL161cApqam6Ovrw2QycejQIV69ekW1WgVA13Wmpqbwer34fD7m5+cNH382WR2gtbUVRVEMwN69ewGwWq0MDg7y7Nkz40+j0ShHjx7FarWyY8cOKpUKjY2N9cPuz3zpuo7ZbCaXy9Hc3IwgCIa+t7eX6elpYzYdP37csB05coR0Oo0kSUaEa9YAwOFwEI1GGRoaqm2YhgbC4TD3799nfHycPXv2GDZJknj69CkOh2Pjcd3R0YHL5UKW5bqPu7u7effuHZIk8e97RBAERFHcGGCz2Whqalpz9K4WOZPJsLKyUgeYmZnZGDA2NlZ31FblyZMnDA8P4/F4qFQqNbZSqUQqlULTtLWvTF3X+fLlC6Ioks1mefDgAZqmUSgUiMfjVCoVJiYmCAaDzM7OcvXqVQYHB+np6aG9vZ3m5maCwSDj4+M16asZdtFolL6+PgRBQNM0Dhw4wNLSEuVyGbvdzqVLl5ienub8+fO0tLQwOjqK3W7n5cuXlEolstns+jeapmkoisLJkycJBALcuXOHmZkZ+vv7UVWVTCZDsVgkEAjQ399v5P3NmzecPn2a+fl5FEUhHA7XAEzFYlEHyGaz3L59m4aGBiRJoru7m9nZWarVKpFIBK/Xy8jICD6fz9i8vLxMMpnk4cOHJJNJbty4QTAYXBuwKqqq8uPHDz5+/MinT5+YnJzkw4cPHDx4kI6OjprNVquVLVu2oOs6fr+f3t7euhSZCoXChg+j79+/Y7PZsFqt//nZ8leA//XDa9MB/wC7Tkud7PApHAAAAABJRU5ErkJggg) icon so you can see the mesh edges clearly.

## Compare various CAD techniques.

A 40x40x5mm box with a triangular hole circumscribed by a circle with a 7.5mm radius:

Current CAD systems:
 * [FreeCAD](./FreeCAD)
 * [fornjot](./fornjot)
 * [csgrs](./csgrs)

Overview:
All three had no issues creating the model itself and exporting to stl format.
Obvoiusly FreeCAD was completely GUI based and can't really compare directly
toi csgrs and fornjot which are code based.

Between csgrs and fornjot I found csgrs was simpler, but that also could
because I'm more familiar with it. But fornjot and FreeCAD both created
"perfect" meshes as opposed to csgrs which had some issues.

Another interesting comparision was the build sizes between fornjot and csgrs.
fornjot's release build was 10MB while csgrs was 0.5MB. The primary reason is
the GUI dependencies in fornjot, In particular the top 4 are all GUI related
and were 40% of the total size from bloat analysis:
```
1.7%  19.2%   4.5MiB naga
1.1%  12.5%   2.9MiB wgpu_core
0.7%   7.2%   1.7MiB wgpu_hal
0.6%   6.5%   1.5MiB winit
```

## Claude

I've created two symlinks from `~/.claude/projects` to the repos `.claude/`
allows me to save the conversations in the repo so everyone can see the
conversations I've had with the bot indepenent of which fornjot dependency
I'm using in  Cargo.toml. You are welcome to do the same and definitely
encourge it if you happen to fork this repo:
```
wink@3900x 25-11-25T01:59:46.550Z:~/data/prgs/3dprinting/box-with-tri-hole/fornjot (main)
$ ls -l ~/.claude/projects/-home-wink-data-prgs-3dprinting-box-with-tri-hole*
lrwxrwxrwx 1 wink users 57 Nov 22 12:09 /home/wink/.claude/projects/-home-wink-data-prgs-3dprinting-box-with-tri-hole -> /home/wink/data/prgs/3dprinting/box-with-tri-hole/.claude
lrwxrwxrwx 1 wink users 57 Nov 24 12:04 /home/wink/.claude/projects/-home-wink-data-prgs-3dprinting-box-with-tri-hole-fornjot -> /home/wink/data/prgs/3dprinting/box-with-tri-hole/.claude
wink@3900x 25-11-25T02:00:08.844Z:~/data/prgs/3dprinting/box-with-tri-hole/fornjot (main
```

## License

Licensed under either of

- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or http://apache.org/licenses/LICENSE-2.0)
- MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

### Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall
be dual licensed as above, without any additional terms or conditions.
