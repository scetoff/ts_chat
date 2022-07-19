Imports System.IO

Module Modules
    Public Function GetRandomImageFilePath(ByVal folderPath As String) As String
        Dim files() As String = Directory.GetFiles(folderPath, "*.png")
        Dim random As Random = New Random()
        Return files(random.Next(0, files.Length - 1))
    End Function
End Module
