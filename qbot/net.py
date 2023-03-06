import logging
import os

import keras.layers
import tensorflow as tf
from tensorflow import keras

logger = logging.getLogger(__name__)

class Residual:

    def __init__(self):
        pass


class Model:

    def __init__(self):
        input_layer = keras.layers.Input((7, 9, 7), name='input')
        layer = keras.layers.ZeroPadding2D((2, 2))(input_layer)
        layer = keras.layers.Conv2D(filters=4, kernel_size=(4, 4), data_format='channels_first', padding="valid", activation='relu')(layer)
        layer = keras.layers.Conv2D(filters=4, kernel_size=(2, 2), data_format='channels_first', padding="valid", activation='relu')(layer)
        layer = keras.layers.BatchNormalization()(layer)
        layer = keras.layers.Activation('relu')(layer)

        for _ in range(8):
            res = layer
            layer = keras.layers.ZeroPadding2D((2, 2))(layer)
            layer = keras.layers.Conv2D(filters=4, kernel_size=(4, 4), data_format='channels_first', padding="valid", activation='relu')(layer)
            layer = keras.layers.Conv2D(filters=4, kernel_size=(2, 2), data_format='channels_first', padding="valid", activation='relu')(layer)
            layer = keras.layers.BatchNormalization()(layer)
            layer = keras.layers.Add()([layer, res])
            layer = keras.layers.Activation('relu')(layer)

        value_head = layer
        value_head = keras.layers.Conv2D(1, (1, 1))(value_head)
        value_head = keras.layers.BatchNormalization()(value_head)
        value_head = keras.layers.Activation('relu')(value_head)
        value_head = keras.layers.Flatten()(value_head)
        value_head = keras.layers.Dense(32)(value_head)
        value_head = keras.layers.Activation('relu')(value_head)
        value_head = keras.layers.Dense(1)(value_head)
        value_head = keras.layers.Activation('tanh', name='value_head')(value_head)

        policy_head = layer
        policy_head = keras.layers.ZeroPadding2D((2, 2))(policy_head)
        policy_head = keras.layers.Conv2D(filters=4, kernel_size=(4, 4), data_format='channels_first', padding="valid", activation='relu')(policy_head)
        policy_head = keras.layers.Conv2D(filters=4, kernel_size=(2, 2), data_format='channels_first', padding="valid", activation='relu')(policy_head)
        policy_head = keras.layers.BatchNormalization()(policy_head)
        policy_head = keras.layers.Flatten()(policy_head)
        policy_head = keras.layers.Dense(7 * 9 * 7 * 9)(policy_head)
        policy_head = keras.layers.Reshape((9, 7, 9, 7))(policy_head)
        policy_head = keras.layers.Activation('softmax', name='policy_head')(policy_head)

        self.model = keras.models.Model(inputs=[input_layer], outputs=[value_head, policy_head])
        self.model.compile(
            optimizer=keras.optimizers.Adadelta(),
            loss=[keras.losses.categorical_crossentropy, keras.losses.mean_squared_error],
            loss_weights=[0.5, 0.5],
            metrics=["accuracy"])

    def load(self, path):
        if os.path.exists(path):
            with open(path, "rt") as f:
                self.model.load_weights(f)
            logger.debug('Loaded weights')
        else:
            logger.debug('No file exists at path to load weights')

    def save(self, path):
        # with open(path, "wt") as f:
        self.model.save(
            path,
            overwrite=True,
            save_format='tf')
        logger.debug('Saved weights')


if __name__ == '__main__':
    model = Model().model
    model.summary()
    print(model.predict([[[[0] * 9] * 7] * 7]))

