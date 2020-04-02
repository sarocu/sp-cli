import pandas
import numpy
from sklearn.linear_model import SVR

class SupportVectorRegression:
    def __init__(self, responseKey, predictorKeys):
        self.predictors = predictorKeys
        self.response = responseKey

    def set_data(self, dataframe):
        """
        Extract the response values into an array and create a matrix for the predictor values, reshaping in the process
        """
        pass

    def fit(self):
        pass

    def predict(self, test_data):
        pass

    def run_tests(self):
        pass