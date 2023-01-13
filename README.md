# libime-history-merge

`libime-history-merge` is a simple CLI for inspecting, merging and editing [libime][libime-repo]
pinyin histories from multiple machines.

## Usage

### Inspecting Your History Data

Your [fcitx5][fcitx5-repo] pinyin input method history typically resides at
`~/.local/share/fcitx5/pinyin/user.history`.  To inspect it, supply its path to
`libime-history-merge`:

```shell
$ libime-history-merge ~/.local/share/fcitx5/pinyin/user.history
```

A pager will be invoked for you to inspect your input history in plain text, passing a
`-n|--no-pager` flag or redirecting output will suppress the pager.

> **NOTE**
> 
> You can check the dumped plain text history data's integrity by comparing it with the
> `libime_history` tool provided by [`libime`][libime-repo]:
>
> ```shell
> $ libime_history ~/.local/share/fcitx5/pinyin/user.history >/tmp/text1
> $ libime-history-merge ~/.local/share/fcitx5/pinyin/user.history >/tmp/text2
> $ diff /tmp/text1 /tmp/text2 && echo $?
>   0
> ```

### Compiling from Plain Text

`libime-history-merge` is able to compile a plain-text file into a history blob.  An example:

```shell
$ # Dump the history content as plain text to ./history.txt
$ cp ~/.local/share/fcitx5/pinyin/user.history original.history
$ libime-history-merge original.history >history.txt
$
$ # Show the history content in plain text
$ cat history.txt
$
$ # Compile the plain-text history back to the binary form and save as ./history.bin
$ libime-history-merge history.txt -o compiled.history
$
$ # Check integrity
$ diff original.history compiled.history && echo $?
  0
```

### Editing Your History Data

`libime-history-merge` also allows you to edit your history entries.  To achive so, simply supply
the `-e|--edit` flag:

```shell
$ libime-history-merge user.history --edit --output ./out.history
```

An editor (possibly specified by the `VISUAL` or `EDITOR` environment variable) will open up with
the history content in plain text.  It is then possible to delete/insert history entries.  After
saving and quiting the editor, `libime-history-merge` will compile the plain text history back to a
[`libime`][libime-repo]-compatible form and save it to the specified output path (`./out.history` in
this case).

### Merging History Data from Multiple Machines

#### Balanced Merge

Pass the `-o|--output` option to specify saving path:

```shell
$ libime-history-merge \
    machine1.history machine2.history machine3.history \
    -o merged.history
$ libime-history-merge merged.history  # Inspect the merged history
```

`libime-history-merge` will mix the input histories uniformly.

#### Weighted Merge

It's also possible to do a weighted merge by passing the weights via the `-w|--weights` option:

```shell
$ libime-history-merge \
    A.history B.history C.history \
    -w 2,5,3 \
    -o merged.history
$ libime-history-merge merged.history  # Inspect the merged history
```

`libime-history-merge` will mix the input histories with the specified priority, in this case, the
first 10 entries of the merged history will be:

```
B[0]
C[0]
B[0]
C[0]
A[0]
B[1]
C[1]
B[1]
C[1]
A[1]
```

where `X[k]` stands for the k-th entry from `X.history`.

> **NOTE**
>
> A balanced merge in the previous example is equiavalent to specifying identical weights to each
> input history, e.g. `-w 3,3,3`.

[fcitx5-repo]: <https://github.com/fcitx/fcitx5>
[libime-repo]: <https://github.com/fcitx/libime>
