CREATE PROCEDURE dbo.GetCustomerDetailsWithTotalPurchases
  @CustomerId INT,
  @Phone VARCHAR(10)
AS
BEGIN
  SET NOCOUNT ON;

  SELECT
    C.CustomerId,
    C.Phone,
    ISNULL(SUM(I.Total), 0) AS TotalPurchases
    FROM
      Customer C
      LEFT JOIN
      Invoice I ON C.CustomerId = I.CustomerId
   WHERE
     C.CustomerId = @CustomerId
   GROUP BY
     C.CustomerId,
     C.FirstName,
     C.LastName,
     C.Company,
     C.Address,
     C.City,
     C.State,
     C.Country,
     C.PostalCode,
     C.Phone,
     C.Email
END;
