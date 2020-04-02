import statsmodels.api as sm
import numpy
import pandas


class VectorAutoRegressive:
    def __init__(self, variables=[]):
        self.variables = variables

    def setData(self, timeseries):
        self.data = timeseries[*self.variables]
        return self

    def fit(self, max_lags=10, ic='aic'):
        self.model = sm.tsa.VAR(self.data)
        self.fitted_model = self.model.fit(maxlags=max_lags, ic=ic)
        return self

    def forecast(self, steps_prior, steps_ahead):
        return self.fitted_model.forecast(priors, steps_ahead)

    def difference(self, shift):
        self.data = self.data.diff(shift)
        return self

    def runTests(self):
        test_set = []

        test_set.append({"test": "lag-order", "value": self.fitted_model.k_ar})

        return test_set
