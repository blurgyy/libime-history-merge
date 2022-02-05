# libime-history-merge

Merge [fcitx5](https://github.com/fcitx/fcitx5) histories from multiple
machines.

## Examples

### Inspect Your History Data

Your fcitx5 pinyin input method history typically resides at
`~/.local/share/fcitx5/pinyin/user.dict`.  To inspect it, supply the path to
it to `libime-history-merge`:

```shell
$ libime-history-merge ~/.local/share/fcitx5/pinyin/user.history
```

A pager will be invoked for you to inspect your input history in plain text,
passing a `-n|--no-pager` flag or redirecting output will suppress the pager.

> **NOTE**: You can check the dumped plain text history data's integrity by
> comparing it with the `libime_history` tool provided by
> [`libime`](https://github.com/fcitx/libime):
>
> ```shell
> $ libime_history ~/.local/share/fcitx5/pinyin/user.history /tmp/text1
> $ libime-history-merge ~/.local/share/fcitx5/pinyin/user.history >/tmp/text2
> $ diff /tmp/text1 /tmp/text2 && echo $?
>   0
> ```

### Merge History Data from Multiple Machines

#### Balanced Merge

Pass the `-o|--output` option to specify saving file:

```shell
$ libime-history-merge \
    machine1.history machine2.history machin3.history \
    -o merged.history
$ libime-history-merge merged.history  # Inspect the merged history
```

#### Weighted Merge

It's also possible to do a weighted merge by passing the weights via the
`-w|--weights` option:

```shell
$ libime-history-merge \
    machine1.history machine2.history machin3.history \
    -w 2,5,3 \
    -o merged.history
$ libime-history-merge merged.history  # Inspect the merged history
```
