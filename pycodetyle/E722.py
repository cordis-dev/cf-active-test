try:
    user = User.objects.get(pk=user_id)
    user.send_mail('Hello world')
except:
    logger.error('An error occurred!')
