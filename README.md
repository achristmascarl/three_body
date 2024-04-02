# ✨ three_body

a very rudimentary simulation of the three-body problem. i was curious how far we could get with just euler's method and a small time step, and it turns out we can get something pretty visually interesting!

![three body problem gif](three_body.gif)

![three body problem image](three_body.png)

i was also curious about what would happen if the polar coordinates of the bodies over time were translated into rgb values and animated over time; the results are below.

**warning**: some of the transitions from this orbit are pretty abrupt, so there may be flashing colors.

![three body problem color gif](color.gif)

## sources

the starting positions for the graphics above are for periodic orbit F<sub>10</sub> from this paper: https://arxiv.org/abs/1805.07980

This is what F<sub>10</sub> looks like when solved with ODE solver dop853 (according to the paper):

<img src="./paper_f10.png" width="300px" alt="F10 from the paper"/>

as you can see, the error in the calculations above grow fairly noticeable after just 2 periods.
