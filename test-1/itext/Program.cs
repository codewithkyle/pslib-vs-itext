using iText.Kernel.Pdf;
using iText.Layout;
using iText.Kernel.Pdf.Canvas;
using iText.Kernel.Colors;
using System.Diagnostics;

namespace ShapeReader
{
    // Define classes for Rectangle and Line
    public class RectangleShape
    {
        public double X { get; set; }
        public double Y { get; set; }
        public double Width { get; set; }
        public double Height { get; set; }
    }

    public class LineShape
    {
        public double X { get; set; }
        public double Y { get; set; }
        public double Length { get; set; }
    }

    // Enum to hold either a Rectangle or Line
    public enum ShapeType { Rectangle, Line }

    public class Shape
    {
        public ShapeType Type { get; set; }
        public RectangleShape? Rect { get; set; }
        public LineShape? Lin { get; set; }
    }

    class Program
    {
        static void Main(string[] args)
        {
            if (args.Length != 1)
            {
                Console.WriteLine("Usage: dotnet run <path_to_text_file>");
                return;
            }

            string filePath = args[0];
            if (!File.Exists(filePath))
            {
                Console.WriteLine($"File not found: {filePath}");
                return;
            }

            // List to store shapes in order
            List<Shape> shapes = new();

            try
            {
                // Read and parse the file line by line
                foreach (var line in File.ReadLines(filePath))
                {
                    var parts = line.Split(' ');
                    switch (parts[^1])
                    {
                        case "rect" when parts.Length == 5:
                            if (double.TryParse(parts[0], out double x1) &&
                                double.TryParse(parts[1], out double y1) &&
                                double.TryParse(parts[2], out double w) &&
                                double.TryParse(parts[3], out double h))
                            {
                                shapes.Add(new Shape
                                {
                                    Type = ShapeType.Rectangle,
                                    Rect = new RectangleShape { X = x1, Y = y1, Width = w, Height = h }
                                });
                            }
                            break;

                        case "line" when parts.Length == 4:
                            if (double.TryParse(parts[0], out double x2) &&
                                double.TryParse(parts[1], out double y2) &&
                                double.TryParse(parts[2], out double l))
                            {
                                shapes.Add(new Shape
                                {
                                    Type = ShapeType.Line,
                                    Lin = new LineShape { X = x2, Y = y2, Length = l }
                                });
                            }
                            break;

                        default:
                            Console.WriteLine($"Skipping malformed line: {line}");
                            break;
                    }
                }

                Stopwatch stopwatch = Stopwatch.StartNew();
                // Create the PDF document and draw shapes
                var pdfWriter = new PdfWriter("output.pdf");
                var pdfDoc = new PdfDocument(pdfWriter);
                var document = new Document(pdfDoc);
                var page = pdfDoc.AddNewPage();
                var canvas = new PdfCanvas(page);
                foreach (var shape in shapes)
                {
                    switch (shape.Type)
                    {
                        case ShapeType.Rectangle when shape.Rect != null:
                            canvas.SetStrokeColor(ColorConstants.BLACK)
                                  .SetLineWidth(1)
                                  .Rectangle((float)shape.Rect.X, (float)shape.Rect.Y, (float)shape.Rect.Width, (float)shape.Rect.Height)
                                  .Stroke();
                            break;

                        case ShapeType.Line when shape.Lin != null:
                            canvas.SetStrokeColor(ColorConstants.BLACK)
                                  .SetLineWidth(1)
                                  .MoveTo((float)shape.Lin.X, (float)shape.Lin.Y)
                                  .LineTo((float)(shape.Lin.X + shape.Lin.Length), (float)shape.Lin.Y)
                                  .Stroke();
                            break;
                    }
                }
                document.Close();
                stopwatch.Stop();
                Console.WriteLine($"Time elapsed: {stopwatch.Elapsed}");
            }
            catch (Exception ex)
            {
                Console.WriteLine(ex.InnerException);
                Console.WriteLine(ex.StackTrace);
                Console.WriteLine($"Error creating PDF: {ex.Message}");
            }
        }
    }
}

