#set text(11pt)

#align(center)[
    #text(17pt)[Hands-on #03: Dynamic Programming]
    
    Eduardo Venturini
]

= Problem 01: Holyday planning

The solution to this problem uses dynamic programming.

For $n' <= n$ and $D' <= D$, let $r_(n', D')$ be the maximum number of attractions the tourist can visit traveling in the first $n'$ cities for exactly $D'$ days.

We notice that $r_(n', D')$ satisfy the following recursive relation:
$ r_(n'+1, D') = max_(i in {0 ... D'}) (r_(n', D'-i) + sum_(j=1)^i a_(n', j)) $
($a_(n', j)$ is the attractions available in the $n'$ city at day $j$).

Using this formula, we can easily calculate $r_(n', D')$ for every value of $n'$ and $D'$, and then return as solution $r_(n, D)$.

We need to calculate $n D$ different values of $r_(n', D')$, and for each value, we need to find the maximum of $D'$ numbers.
Since we can precalculate all the sums $sum_(j=1)^i a_(n', j)$ with $n D$ operations, the total time complexity is $Theta(n D^2)$.

The space complexity is $Theta(D)$: we just need to store $r_(n', D')$ for $0 < D' <= D$, starting from $n' = 0$ and updating the vector at each iteration of $n'$ until $n' = n$.

= Problem 02: Xmas Lights

The solution to this problem uses dynamic programming.

Let $s_(n')$ be the color of the $n'$-th house.

For $n' <= n$ and $0 <= D' <= 3$, let $r_(n', D')$ be the number of $D'$-uples of houses among the first $n'$ houses and among all colorings of such $n'$ houses, such that the first house (if $D' >= 1$) is red, the second one (if $D' >= 2$) is white and the third one (if $D' >= 3$) is green.

We notice that $r_(n', D')$ satisfy the following recursive relations:
$
r_(0, 0) &= 1
#h(1cm)
& r_(n'+1, 0) &= cases(
    3 r_(n', 0) quad &"if" s(n'+1) = "X",
    r_(n', 0) quad &"if" s(n'+1) != "X",
)
\

r_(0, 1) &= 0#h(1cm)
& r_(n'+1, 1) &= cases(
    r_(n', 1) + r_(n', 0) quad &"if" s(n'+1) = "R",
    3 r_(n', 1) + r_(n', 0) quad &"if" s(n'+1) = "X",
    r_(n', 1) quad &"if" s(n'+1) != "R, X",
)
\

r_(0, 2) &= 0
& r_(n'+1, 2) &= cases(
    r_(n', 2) + r_(n', 1) quad &"if" s(n'+1) = "W",
    3 r_(n', 2) + r_(n', 1) quad &"if" s(n'+1) = "X",
    r_(n', 2) quad &"if" s(n'+1) != "W, X",
)
\

r_(0, 3) &= 0
& r_(n'+1, 3) &= cases(
    r_(n', 3) + r_(n', 2) quad &"if" s(n'+1) = "G",
    3 r_(n', 3) + r_(n', 2) quad &"if" s(n'+1) = "X",
    r_(n', 3) quad &"if" s(n'+1) != "G, X",
)
$

Using this formula, we can easily calculate $r_(n', D')$ for every value of $n'$ and $D'$, and then return as solution $r_(n, 3)$.

Both time and space complexity are $Theta(n)$, although space complexity can easily be reduced to $Theta(1)$ by keeping the values of $r_(n', D')$ for only the current $n'$.

We also notice that this approach is easily generalizable to arbitrary subsequences of houses and not only RWG.