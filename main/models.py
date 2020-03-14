from django.db import models


class CategoryChoice(models.IntegerChoices):
    RESTAURANT = 0
    Bar = 1
    COFFEE_SHOP = 2
    CLOTHING = 3


class Business(models.Model):
    name = models.CharField(max_length=30)
    category = models.IntegerChoices(choicdes=CategoryChoice.choices)

    # Address info
    # todo: table for cities etc?
    country = models.IntegerField()  # ICAO country code. (Which format?)
    city = models.CharField(max_length=30)
    street = models.CharField(max_length=50)
    address_num = models.CharField(max_length=30)
    postal_code = models.CharField(max_length=30)


class Issues(models.Model):
    business = models.ForeignKey(Business, on_delete=models.CASCADE)
    title = models.CharField(max_length=50)
    text = models.TextField()
