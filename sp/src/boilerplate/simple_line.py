from bokeh.plotting import figure, output_file, show
from bokeh.models import ColumnDataSource
import pandas


class SimpleLineChart:
    def __init__(
        self,
        title,
        xlabel,
        ylabel,
        save_as="output/line.html",
        date_axis=False,
        **kwargs
    ):
        self.title = title
        self.source = None

        if date_axis:
            self.viz = figure(
                title=title,
                x_axis_label=xlabel,
                y_axis_label=ylabel,
                x_axis_type="datetime",
                **kwargs,
            )
        else:
            self.viz = figure(
                title=title, x_axis_label=xlabel, y_axis_label=ylabel, **kwargs
            )

        output_file(save_as)

    def set_data(self, dataframe):
        self.source = ColumnDataSource(data=dataframe)
        return self

    def add_marker(self, xvar, yvar, **kwargs):
        glyph = kwargs.pop("marker")
        marker_render = getattr(self.viz, glyph)
        marker_render(xvar, yvar, source=self.source, **kwargs)

        return self

    def add_line(self, xvar, yvar, **kwargs):
        if "marker" in kwargs:
            glyph = kwargs.pop("marker")
            marker_render = getattr(self.viz, glyph)
            marker_render(xvar, yvar, source=self.source, **kwargs)

        self.viz.line(xvar, yvar, source=self.source, **kwargs)

        return self

    def render(self):
        show(self.viz)
