/* wow */

int wow() {}

int neg1()
{
    return -1;
}

int a(int apple)
{
    return apple;
}

int b(int banana, int pear)
{
    if (pear > a(banana))
        println(pear - banana);
    else if (banana > pear)
        println(banana - pear);
    else if (pear == banana && (pear - banana) == 0)
        println(neg1());
    wow();
    return (a(banana) * pear) / 2;
}

/*
    this is the main function.
*/
int main()
{
    int int_, else_;
    int_ = 10;
    else_ = 5;
    while (int_ > 0 && else_ < 15)
    {
        b(int_, else_);
        int_ = int_ + neg1();
        else_ = else_ - neg1();
    }
}