import cv2
vidcap = cv2.VideoCapture('vid.mp4')
success,image = vidcap.read()
count = 0
while success:
  cv2.imwrite("frames/frame%d.jpg" % count, image)
  success,image = vidcap.read()
  count += 1
