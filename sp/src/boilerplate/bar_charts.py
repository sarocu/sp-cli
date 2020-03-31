from bokeh.plotting import figure, output_file, show
from bokeh.models import ColumnDataSource
import pandas


class BarCharts:
    def __init__(self, title, save_as="output/bars.html", date_axis=False, **kwargs):
        self.title = title
        self.source = None

        if date_axis:
            self.viz = figure(title=title, x_axis_type="datetime", **kwargs,)
        else:
            self.viz = figure(title=title, **kwargs)

        output_file(save_as)

    def set_data(self, dataframe):
        self.source = ColumnDataSource(data=dataframe)
        return self

    def legend_props(self, **kwargs):
        for keyword in kwargs:
            setattr(self.viz.legend, str(keyword), str(kwargs[keyword]))

        return self

    def add_vertical_bar(self, xvar, yvar, **kwargs):
        self.viz.vbar(x=xvar, top=yvar, **kwargs, source=self.source)

        return self

    def add_horizontal_bar(self, xvar, yvar, **kwargs):
        self.viz.hbar(y=xvar, right=yvar, **kwargs, source=self.source)

        return self

    def render(self):
        show(self.viz)
