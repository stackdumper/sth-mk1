# from angles import Angles
import tensorflow as tf
import numpy as np


def main():
    (x_train, y_train), (x_test, y_test) = load_data()

    # model = train_model(x_train, y_train)
    # model.save('test.model')

    model = tf.keras.models.load_model('test.model')
    predictions = model.predict(x_test)
    print(np.argmax(predictions[0]))


def load_data():
    (x_train, y_train), (x_test, y_test) = tf.keras.datasets.mnist.load_data()

    x_train = tf.keras.utils.normalize(x_train, axis=1)
    x_test = tf.keras.utils.normalize(x_test, axis=1)

    return (x_train, y_train), (x_test, y_test)


def train_model(x, y):
    model = tf.keras.models.Sequential()
    model.add(tf.keras.layers.Flatten())
    model.add(tf.keras.layers.Dense(128, activation=tf.nn.relu))
    model.add(tf.keras.layers.Dense(128, activation=tf.nn.relu))
    model.add(tf.keras.layers.Dense(10, activation=tf.nn.softmax))

    model.compile(
        optimizer='adam',
        loss='sparse_categorical_crossentropy',
        metrics=['accuracy'],
    )

    model.fit(x, y, epochs=3)

    # val_loss, val_acc = model.evaluate(x_test, y_test)
    # print(val_loss, val_acc)

    return model


if __name__ == '__main__':
    main()
