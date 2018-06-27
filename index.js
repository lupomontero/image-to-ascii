import { from_data } from './build/release/image_to_ascii';


export const fromData = (width, height, data, mode = "plain") =>
  from_data(width, height, data, mode);


export const fromCanvas = (canvas, mode = "plain") => {
  const context = canvas.getContext('2d');
  const { width, height } = canvas;
  const { data } = context.getImageData(0, 0, width, height);
  return fromData(width, height, data, mode);
};
