import statsmodels.api as sm
import numpy


class VectorAutoRegressive:
    def __init__(self, maxLags, criterion):
        self.lags = maxLags
        self.ic = criterion

    def setData(self, timeseries):
        self.model = sm.tsa.VAR(timeseries)

    def results(self):
        self.fit = self.model.fit(maxlags=self.lags, ic=self.ic)

    def runTests(self):
        test_set = []

        test_set.append({"test": "lag-order", "value": self.fit.k_ar})

        return test_set
