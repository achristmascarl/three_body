# ✨ three_body

a very rudimentary simulation of the three-body problem. i was curious how far we could get with just euler's method and a small time step, and it turns out we can get something pretty visually interesting!

![three body problem gif](three_body.gif)

![three body problem image](three_body.png)

the starting positions for the graphics above are for periodic orbit F<sub>10</sub> from this paper: https://arxiv.org/abs/1805.07980

This is what F<sub>10</sub> looks like when solved with ODE solver dop853 (according to the paper):
![f10 from paper](paper_f10.png)

as you can see, the error is fairly noticeable after just 2 periods.
