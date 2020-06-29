# rusticode

是树和链表，快跑！


#### 初心

刚用`rust`写`leetcode`的时候，一遇到树和链表的题，就满地`clone`，才能通过编译。后面对`rust`熟悉了之后，就找到了一些避免多余副本的方法。因为每一个副本都需要消耗内存，克隆的操作可能也会比较耗时。如果一直`clone`，速度和效率就大打折扣了。相比隔壁的`C`，虽然有了自动内存管理，但却是以拷贝作为代价。

概括地说，避免不必要`clone`的方法是：

+ 使用引用，适用于不需要修改原数据结构的场景，比如树的深度，链表的长度。
+ 使用`take`，适用于需要修改原数据结构的场景，比如重建二叉树。

对于那些正在用`rust`写题的同学，可能树和链表的代码是最有参考性的。

+ [tree.rs](leetcode/src/tree.rs)
+ [ilist.rs](leetcode/src/ilist.rs)

#### misc

你可以通过`cargo`跑测试。这里面的测试并没有涵盖所有的题目。一开始，每做一题我都会在`lib.rs`写对应的测试，后来觉得太麻烦就没有写了。
```sh
cargo test
```

如果你想在`rust`中写`C`，这个项目也有一点点的参考价值。你可以看看`lib.rs`和`leetcode.c`是怎么交互的。



#### wishes

愿凡有所得，皆能自利利他。