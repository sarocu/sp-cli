import pandas
import numpy
from sklearn.linear_model import LinearRegression


class SimpleLinear:
    def __init__(self, predictorKey, responseKey):
        self.predictor = predictorKey
        self.response = responseKey

    def set_data(self, dataframe):
        predictorArray = dataframe[self.predictor]
        responseArray = dataframe[self.response]
        self.x = predictorArray.values.reshape(-1, 1)
        self.y = responseArray.values.reshape(-1, 1)
        return self

    def fit(self):
        self.fitted_model = LinearRegression().fit(self.x, self.y)
        return self

    def predict_one(self, value):
        return self.fitted_model.predict(numpy.array([value]).reshape(1, -1))

    def predict(self, value_list):
        return self.fitted_model.predict(numpy.array(value_list).reshape(-1, 1))

    def run_tests(self):
        test_set = []

        # Standard score method, includes R2:
        test_set.append(
            {"test": "r2", "results": self.fitted_model.score(self.x, self.y)}
        )
        return test_set
