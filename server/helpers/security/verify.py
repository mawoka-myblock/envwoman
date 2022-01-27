import smtplib
import ssl
import os
from email.mime.multipart import MIMEMultipart
from helpers.config import col, settings
from email.mime.text import MIMEText


async def send_mail(email: str):
    if settings.skip_email_verification:
        await col("users").update_one({"email": email}, {"$set": {"verified": True}})
    else:
        random_key = os.urandom(60).hex()
        user = await col("users").find_one_and_update({"email": email, "verified": False},
                                                      {"$set": {"verified": random_key}})
        msg = MIMEMultipart('alternative')
        msg['Subject'] = "Verify your account"
        msg['From'] = settings.mail_address
        msg['To'] = email

        text = f"""
        Hey {user["email"]}!
        Please visit the following page to verify your account: {settings.root_address}/api/v1/users/users/verify/{random_key}
        Cheers!
        """
        msg.attach(MIMEText(text, "plain"))
        context = ssl.SSLContext(ssl.PROTOCOL_TLS)
        server = smtplib.SMTP(host=settings.mail_server, port=settings.mail_port)
        server.ehlo()
        server.starttls(context=context)
        server.ehlo()
        server.login(settings.mail_username, settings.mail_password)
        server.sendmail(settings.mail_address, email, msg.as_string())

